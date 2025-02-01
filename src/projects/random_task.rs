extern crate maud;
use maud::{html, Markup};
use std::error::Error;

pub fn random_task() -> Result<Markup, Box<dyn Error>> {
    let button_id = "task_select_button";
    let input_id = "task_upload_input";
    let task_display_id = "task_display";
    let file_name_display_id = "file_name_display";
    let random_task_html = html! {
        h1{"Random Task"}
        p{
            "A little utility to pick a task from an arbitrarily nested list of tasks. The repo is "
                a href="https://github.com/TomPridham/random-task"{"here"}
            ". I made this because I get stuck trying to decide what thing to do sometimes. I couldn't use a normal random number generator because I have like 100 house projects and I didn't want them to dominate the list entirely. So, I made this to select tasks more fairly from a nested list."
        }


        div.col{
            label.button for=(input_id){"upload task list"}
            input type="file" id=(input_id){}
            span aria-live="polite" id=(file_name_display_id){"No file selected"}
            button.button id=(button_id){"get random task"}
            p id=(task_display_id){}
        }
        script type="module"{
"
import init, {get_task,build_nested_task_list }from './random_task.js';
await init()

const inputId = '"(input_id)"';
const buttonId = '"(button_id)"';
let tasks = [];
let chosenTask = '????';
const uploadHandler = function(event) {
    const file = event.target.files[0];
    const reader = new FileReader();
    document.getElementById('"(file_name_display_id)"').textContent = `Selected file: ${file.name}`;
    reader.onload = function() {
        tasks = build_nested_task_list(reader.result.split('\\n'), '  ');
    };
    reader.onerror = function() {
        showMessage('broke');
    };
    reader.readAsText(file);
}
const getTaskHandler = function() {
    if (tasks.length === 0){
        return
    }
    chosenTask = get_task(tasks);
    document.getElementById('"(task_display_id)"').innerText= chosenTask
}
document.getElementById('"(input_id)"').addEventListener('change', uploadHandler)
document.getElementById('"(button_id)"').addEventListener('click', getTaskHandler)
"
        }

    };
    Ok(random_task_html)
}
