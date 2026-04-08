mod parsing;
use parsing::Lexer;
use parsing::{ AstNode, Command };
pub use executer::{ exec, check_background_jobs, Cmd };
use std::io;

pub fn evaluate(user_input: &str) {
    let mut cmd_line: String = user_input.to_string().trim().to_string();
    if !cmd_line.ends_with(";") {
        cmd_line += ";";
    }
    let lexer_tokens = Lexer::new(&cmd_line);
    let ast_data = AstNode::new(lexer_tokens);
    for sub_vector in ast_data {
        for node in sub_vector {
            match node {
                AstNode::Pipeline(commands, bg) => {
                    let mut cmds: Vec<Cmd> = Vec::new();
                    for mut c in commands {
                        c.is_background = bg;
                        cmds.push(to_cmd(c));
                    }
                    exec(cmds);
                }
                AstNode::Command(command) => {
                    if command.program.len() != 0 {
                        exec(vec![to_cmd(command)]);
                    }
                }
                AstNode::None => {}
            }
        }
    }
}

fn to_cmd(command: Command) -> Cmd {
    Cmd {
        cmd: command.program,
        args: command.arguments,
        stdin: Box::new(io::stdin()),
        stdout: Box::new(io::stdout()),
        stderr: Box::new(io::stderr()),
        is_background: command.is_background,
    }
}
