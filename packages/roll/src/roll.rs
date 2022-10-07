use std::iter::repeat;

use atoms::{loading,Heading2,HeadingDescription};
use firestore_hooks::use_document;
use model::Room;
use yew::{function_component, html, Callback, Properties, Html};
use layouting::{FixToBottom};

use crate::use_roll::use_roll;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub room_id: String,
}

fn icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-7 h-7">
            <path fill-rule="evenodd" d="M7.5 6a4.5 4.5 0 119 0 4.5 4.5 0 01-9 0zM3.751 20.105a8.25 8.25 0 0116.498 0 .75.75 0 01-.437.695A18.683 18.683 0 0112 22.5c-2.786 0-5.433-.608-7.812-1.7a.75.75 0 01-.437-.695z" clip-rule="evenodd" />
        </svg>
    }
}


fn check_icon() -> Html {
    html! {
        <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-9 h-9">
            <path fill-rule="evenodd" d="M19.916 4.626a.75.75 0 01.208 1.04l-9 13.5a.75.75 0 01-1.154.114l-6-6a.75.75 0 011.06-1.06l5.353 5.353 8.493-12.739a.75.75 0 011.04-.208z" clip-rule="evenodd" />
        </svg>
    }
}

#[function_component(RollButton)]
pub fn roll(props: &Props) -> Html {
    let room = use_document::<Room>(&(), props.room_id.as_str());
    let roll = use_roll(props.room_id.as_str());
    match room {
        firestore_hooks::DataFetchState::Loading => loading(),
        firestore_hooks::DataFetchState::Loaded(room) => {
            let rule = room.rule.unwrap();
            
            html! {
                <section>
                    <Heading2>{"役職一覧"}</Heading2>
                    <HeadingDescription>{"参加者:5 / 役職:10"}</HeadingDescription>
                    <div class="w-80 h-96 mx-auto mt-12">
                        <ul class="flex flex-col gap-5 mt-4">
                            {
                                for rule.roles.iter().map(|roll| {
                                    html! {
                                        <li class="flex text-black pb-1 border-solid border-b border-line">
                                            <span class="text-xl mr-3">
                                                {roll.name.as_str()}
                                            </span>
                                            <span class="text-black-light flex">
                                                {for repeat(icon()).take(roll.number)}
                                            </span>
                                        </li>
                                    }
                                })
                            }
                        </ul>
                        
                        {
                            match roll {
                                Some(roll) => {
                                    html! {
                                        <FixToBottom>
                                            <div class="m-auto flex justify-center">
                                                <button class={"animate-bounce bg-feature hover:bg-feature-light text-white py-3 px-3 text-lg rounded-full"} onclick={Callback::from(move |_| roll())}>
                                                    {check_icon()}
                                                </button>
                                            </div>
                                        </FixToBottom>
                                    }
                                },
                                None => loading(),
                            }
                        }
                    </div>
                </section>
            }

            
        },
    }

    
}