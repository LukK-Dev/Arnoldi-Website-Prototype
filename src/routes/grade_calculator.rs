use leptos::*;
use leptos_meta::{provide_meta_context, Title};

use crate::components::{footer::Footer, navbar::Navbar, background::Background};

#[component]
pub fn GradeCalculator() -> impl IntoView {
    provide_meta_context();

    view! {
        <Title text="Arnoldi MVP | Home"/>

        <div class="flex flex-col h-[50vh]">
            <Navbar/>
            <div class="text-white h-[50vh] w-full flex flex-col justify-center items-center">
                <h1 class="font-semibold lg:text-8xl text-6xl mb-4">Notenrechner</h1>
            </div>
        </div>

        <Background />

        <main class="flex justify-center items-center">
            <div class="w-full h-screen bg-white" style="">
                <Rechner/>
            </div>
        </main>

        <Footer />
    }
}

#[component]
fn Rechner() -> impl IntoView {
    // create a signal to hold the value
    let (max_be, set_max_be) = create_signal("0".to_string());
    let (achieved_be, set_achieved_be) = create_signal("0".to_string());
    let (percentage, set_percentage) = create_signal("0".to_string());
    let (grade, set_grade) = create_signal("0".to_string());

    let (mark, set_mark) = create_signal(true);

    fn percent(ach_points: f32 , max_points: f32) -> f32{
        let percent = ach_points / max_points * 100.0;
        return percent.round();
    }

    fn percent_to_grade(pcent: f32) -> u32 {
        let mut ach_grade = 0;
        if pcent >= 95.0{
            ach_grade = 1;
        } else if pcent >= 80.0{
            ach_grade = 2;
        } else if pcent >= 65.0{
            ach_grade = 3;
        } else if pcent >= 50.0{
            ach_grade = 4;
        } else if pcent >= 25.0{
            ach_grade = 5;
        } else {
            ach_grade = 6;
        }
        return ach_grade;
    }

    fn percent_to_points(pcent: f32) -> u32 {
        let mut ach_grade = 0;
        if pcent >= 95.0{
            ach_grade = 15;
        } else if pcent >= 90.0{
            ach_grade = 14;
        } else if pcent >= 85.0{
            ach_grade = 13;
        } else if pcent >= 80.0{
            ach_grade = 12;
        } else if pcent >= 75.0{
            ach_grade = 11;
        } else if pcent >= 70.0{
            ach_grade = 10;
        } else if pcent >= 65.0{
            ach_grade = 9;
        } else if pcent >= 60.0{
            ach_grade = 8;
        } else if pcent >= 55.0{
            ach_grade = 7;
        } else if pcent >= 50.0{
            ach_grade = 6;
        } else if pcent >= 45.0{
            ach_grade = 5;
        } else if pcent >= 40.0{
            ach_grade = 4;
        } else if pcent >= 33.0{
            ach_grade = 3;
        } else if pcent >= 27.0{
            ach_grade = 2;
        } else if pcent >= 20.0{
            ach_grade = 1;
        } else {
            ach_grade = 0;
        }
        return ach_grade;
    }

    fn adjust_result(option: bool, percent: f32) -> String{
        if option{
            return percent_to_grade(percent).to_string()
        }
        else{
            return percent_to_points(percent).to_string()
        }
    }

    view! {
        <div class="p-[10px]">
            <input type="radio" id="css" name="name" value="CSS" checked="checked"
                on:input=move |ev| {
                    set_mark(event_target_checked(&ev));
                    set_grade(adjust_result(mark.get(), percentage.get().parse::<f32>().unwrap()));
                }
                />
                <label for="css" class="p-[5px] pr-[20px]">Note</label>
                <input type="radio" id="javascript" name="name" value="JavaScript" 
                    on:input=move |ev| {
                        if (event_target_checked(&ev) == true) {
                            set_mark(false)
                        }
                        set_grade(adjust_result(mark.get(), percentage.get().parse::<f32>().unwrap()));
                    }
                />
                <label for="javascript" class="p-[5px]">Notenpunkte</label>

            <h2>Zu erreichende Punktzahl:</h2>
            <input type="number" class="text-slate-600" min="1" max="125" step="1"
                on:input=move |ev| {
                    set_max_be(event_target_value(&ev));
                    set_percentage((percent(achieved_be.get().parse::<f32>().unwrap(), max_be.get().parse::<f32>().unwrap())).to_string());
                    set_grade(adjust_result(mark.get(), percentage.get().parse::<f32>().unwrap()));
                }

                prop:value=max_be
            />
            <h2>erreichte Punktzahl:</h2>
            <input type="number" class="text-slate-600" min="1" max="125" step="1"
                on:input=move |ev| {
                    set_achieved_be(event_target_value(&ev));
                    set_percentage((percent(achieved_be.get().parse::<f32>().unwrap(), max_be.get().parse::<f32>().unwrap())).to_string());
                    set_grade(adjust_result(mark.get(), percentage.get().parse::<f32>().unwrap()));
                }

                prop:value=achieved_be
            />

            <p>"Es wurde Note " {grade} " erreicht mit " {percentage} "%"</p>
        </div>
    }
}
