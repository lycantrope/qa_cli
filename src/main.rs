use itertools::Itertools;
use promptly::{prompt, prompt_default};
use colored::*;

static QEUSTION_STR:&str  = include_str!("../questions.json");  



#[derive(serde::Deserialize, Debug, Clone)]
struct Quiz{
    pub prompt:String,
    pub options: Vec<String>,
    pub ans:usize,
}


impl Quiz{
    fn get_options(&self) -> String{
        if self.options.is_empty(){
            "".into()
        } else{
            let spacer = "\n".to_string();
            let res:String = self.options.iter().enumerate().map(|(i, o)|{
                format!("{}. {}", i+1, o)
            }).intersperse(spacer)
            .collect();
            res

        }
    }
}


fn main() -> anyhow::Result<()> {
    let quiz:Vec<Quiz> = serde_json::from_str(QEUSTION_STR)?;
    // print!("\x1B[2J\x1B[1;1H");
    println!("{}", "Welcome!".green());
    let mut results:Vec<(bool, usize)> = Vec::with_capacity(quiz.len());

    for (i, q) in quiz.iter().enumerate(){
        let n_opt = q.options.len();
        loop{

            let ans = loop{
                println!("---------------------------------");
                println!("{}", format!("Q{}: {}", i+1, q.prompt).green().bold());
                println!("{}",q.get_options().blue());
                let ans:usize = prompt(format!("Your Answer (1-{})", n_opt))?;
            if ans > 0 && ans <= n_opt{
                break ans;
            }
            // print!("\x1B[2J\x1B[1;1H");
            println!("---------------------------------");
            println!("{}", format!("Invalid Input: {}", ans).on_red());
        };
        
            if ans == q.ans{
                println!("{}", "Yes.".green().bold());
                results.push((true, ans));
                break
            }else{
                println!("{}", "No.".red().bold());
                let redo:bool = prompt_default("Do you want to retry this question?", false)?;
                if !redo{
                    results.push((false, ans));
                    break
                }
            }
        }
    }
    let spacer = "\n".to_string();
    let results_str:String = results.into_iter().enumerate().map(|(i, (is_correct, ans))|{
        let fmt_str= if is_correct{
            format!("{}(O)", ans).green().bold()
        }else{
            format!("{}(X)", ans).red().bold()
        };
        format!("Q{}: {}",i+1, fmt_str)
    }).intersperse(spacer)
    .collect();
    loop{
        print!("\x1B[2J\x1B[1;1H");
        println!("{}", "#### Summary ####".green().bold());
        println!("{}", results_str);
        let exit = prompt_default("Exit?", true)?;
        if exit{
            break
        }
    }
    Ok(())
}
