use sycamore::prelude::*;

struct Project {
    name: String,
    url: String,
    picture: String,
    description: String
}

#[component]
pub fn send_view<G: Html>(projects: Vec<Project>, cx: Scope) -> View<G> {

    let rl: u32 = 0;

    view! {
        div(class="project_container") {
            span() {
                for i in projects {
                    if i < 2 {
                        return;
                    }
                    projects[i].name;
                    projects[i].url;
                    projects[i].picture;
                    projects[i].description;
                }
            }
        }
    }

}

fn create_project(name: String, url: String, picture: String, description: String) -> Vec<Project> {
    let projects: Vec<Project> = "";
    projects = [
        name: "test",
        url: "https://github.com/awtgerry/test",
        picture: "this",
        description "test",
    ];
    projects
}
