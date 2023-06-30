use iced::{Sandbox, Settings};


struct TodoList {
    tasks: Vec<Task>,
    being_edited: Option<u32>,
}

enum Status {
    Todo, Doing, Done
}

struct Task {
    id: u32,
    description: String,
    status: Status,
}

#[derive(Debug)]
enum Message {
}

impl Sandbox for TodoList {
    type Message = Message;

    fn new() -> Self {

        let mut dummy_tasks = Vec::new();
        dummy_tasks.extend([
            Task {
                id: 1, description: "Create a login form: Design a login form with fields for username and password. Implement validation checks and provide appropriate error messages for incorrect credentials.".into(), status: Status::Todo,
            },
            Task {
                id: 2, description: "Build a countdown timer: Develop a countdown timer that starts from a specified time and updates the UI in real-time. Include features like setting the timer duration and displaying a notification when the timer reaches zero.".into(), status: Status::Todo,
            },
            Task {
                id: 3, description: "Implement a draggable element: Create a draggable element that users can click and drag within a defined area. Ensure smooth movement and update the element's position dynamically.".into(), status: Status::Doing,
            },
            Task {
                id: 4, description: "Design a responsive navbar: Develop a responsive navigation bar that adapts to different screen sizes. Include a hamburger menu icon for mobile devices and ensure smooth transitions between menu states.".into(), status: Status::Doing,
            },
            Task {
                id: 5, description: "Add pagination to a data table: Enhance an existing data table by implementing pagination. Display a limited number of rows per page and provide navigation controls to switch between pages.".into(), status: Status::Done,
            },
        ]);

        Self { 
            tasks: Vec::new(), 
            being_edited: Some(3), 
        }
    }

    fn title(&self) -> String {
        "To Do List".into()
    }

    fn update(&mut self, message: Self::Message) {
        match message {}
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        
        // three columns, evenly spaced, with headings
        // tasks as boxes within
        // if task is being edited, render with editor
        todo!()
    }
}

fn main() -> iced::Result {
    TodoList::run(Settings::default())
}
