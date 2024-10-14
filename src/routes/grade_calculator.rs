use leptos::*;
use leptos_meta::{provide_meta_context, Title};

use crate::components::navbar::Navbar;

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

        <div class="fixed inset-0 z-[-1] overflow-hidden">
            <img src="/school.jpg" class="object-center w-full h-full object-cover pointer-events-none brightness-50 blur-sm scale-105"/>
        </div>

        <main class="flex justify-center items-center">
            <div class="w-full h-screen bg-white" style="">
                <Rechner/>
            </div>
        </main>
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

    fn percent_to_grade(pcent: f32) -> i32 {
        let mut ach_grade = 0;
        if pcent > 95.0{
            ach_grade = 1;
        } else if pcent > 80.0{
            ach_grade = 2;
        } else if pcent > 65.0{
            ach_grade = 3;
        } else if pcent > 50.0{
            ach_grade = 4;
        } else if pcent > 25.0{
            ach_grade = 5;
        } else {
            ach_grade = 6;
        }
        return ach_grade;
    }

    view! {
        <div class="p-[10px]">
            <input type="radio" id="css" name="name" value="CSS" checked="checked"
                on:input=move |ev| {
                    set_mark(event_target_checked(&ev))
                }
                />
                <label for="css" class="p-[5px] pr-[20px]">Note</label>
                <input type="radio" id="javascript" name="name" value="JavaScript" 
                    on:input=move |ev| {
                        if (event_target_checked(&ev) == true) {
                            set_mark(false)
                        }
                    }
                />
                <label for="javascript" class="p-[5px]">Notenpunkte</label>
            <p>"Name is: " {mark}</p>

            <h2>Zu erreichende Punktzahl:</h2>
            <input type="number" class="text-slate-600" min="1" max="125" step="1"
                on:input=move |ev| {
                    set_max_be(event_target_value(&ev));
                    set_percentage((percent(achieved_be.get().parse::<f32>().unwrap(), max_be.get().parse::<f32>().unwrap())).to_string());
                    set_grade(percent_to_grade(percentage.get().parse::<f32>().unwrap()).to_string())
                }

                prop:value=max_be
            />
            <h2>erreichte Punktzahl:</h2>
            <input type="number" class="text-slate-600" min="1" max="125" step="1"
                on:input=move |ev| {
                    set_achieved_be(event_target_value(&ev));
                    set_percentage((percent(achieved_be.get().parse::<f32>().unwrap(), max_be.get().parse::<f32>().unwrap())).to_string());
                    set_grade(percent_to_grade(percentage.get().parse::<f32>().unwrap()).to_string())
                }

                prop:value=achieved_be
            />

            <p>"Du hast Note " {grade} " erreicht mit " {percentage} "%"</p>
        </div>
    }
}
