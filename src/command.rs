use std::{
    io::{self, BufRead, BufReader, Write},
    process::{Child, ChildStdout, ExitStatus},
    thread::{self, JoinHandle},
};

use crossbeam::channel::{unbounded, Receiver, Sender};

pub enum ServerResult {
    Continue,
    Stopped,
    Restart,
}

pub struct ServerCommand {
    pub command: Child,
    pub text: Vec<String>,

    pub stdin_thread: JoinHandle<()>,
    pub stdin_receiver: Receiver<String>,

    pub stdout_thread: JoinHandle<()>,
    pub stdout_receiver: Receiver<String>,

    pub history: usize,
    pub running: bool,
}

impl ServerCommand {
    pub fn new(mut command: Child, history: usize) -> Self {
        let (in_send, in_recv) = unbounded();
        let (out_send, out_recv) = unbounded();

        let stdin_thread = thread::spawn(|| input_catcher(in_send));

        let stdout = command.stdout.take().unwrap();
        let stdout_thread = thread::spawn(|| output_catcher(out_send, stdout));

        Self {
            command,
            text: vec![],

            stdin_thread,
            stdin_receiver: in_recv,

            stdout_thread,
            stdout_receiver: out_recv,

            history,
            running: true,
        }
    }

    pub fn handle(&mut self) -> anyhow::Result<ServerResult> {
        let commands = self.handle_input();

        if !self.running {
            for command in commands {
                let command = command.trim();

                if command == "start" {
                    println!("Starting Process!");
                    return Ok(ServerResult::Restart);
                }
            }

            Ok(ServerResult::Stopped)
        } else {
            for command in commands {
                let command = command.trim();
                if command == "ctrl-c" {
                    println!("Killing Process!");
                    self.send_kill();
                    return Ok(ServerResult::Stopped);
                }
                self.send_string(command.to_string())?;
            }

            self.handle_output();

            if let Some(status) = self.check_complete() {
                println!("Process Exited with Status: {status}");
                return Ok(ServerResult::Stopped);
            }

            Ok(ServerResult::Continue)
        }
    }

    pub fn handle_input(&mut self) -> Vec<String> {
        let mut inputs = vec![];

        while let Ok(item) = self.stdin_receiver.try_recv() {
            if item.is_empty() {
                continue;
            }
            inputs.push(item)
        }

        inputs
    }

    pub fn handle_output(&mut self) {
        while let Ok(receive) = self.stdout_receiver.try_recv() {
            print!("{receive}");
            self.text.push(receive);

            if self.text.len() > self.history {
                self.text = self.text[(self.text.len() - self.history)..self.history].to_vec();
            }
        }
    }

    pub fn send_kill(&mut self) {
        self.command.kill().expect("Cannot kill process");
    }

    pub fn check_complete(&mut self) -> Option<ExitStatus> {
        if let Ok(code) = self.command.try_wait() {
            return code;
        }
        None
    }

    pub fn send_string(&mut self, mut write: String) -> std::io::Result<()> {
        write.push('\n');

        self.text.push(write.clone());

        if self.check_complete().is_some() {
            return Ok(());
        }

        self.command
            .stdin
            .as_mut()
            .unwrap()
            .write_all(write.as_bytes())?;
        self.command.stdin.as_mut().unwrap().flush().unwrap();

        Ok(())
    }
}

pub fn output_catcher(msg_link: Sender<String>, stdout: ChildStdout) {
    let mut reader = BufReader::new(stdout);

    loop {
        let mut output = String::new();
        match reader.read_line(&mut output) {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }

        if output.trim() == "" {
            continue;
        }

        match msg_link.send(output) {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
    }
}

pub fn input_catcher(msg_link: Sender<String>) {
    let mut reader = BufReader::new(io::stdin());

    loop {
        let mut output = String::new();
        match reader.read_line(&mut output) {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }

        match msg_link.send(output) {
            Ok(_) => {}
            Err(_) => {
                break;
            }
        }
    }
}
