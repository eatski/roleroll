use firestore_hooks::use_collection_sync;
use model::{Rule, Role, SetRule, MemberJSON};
use yew::{function_component, html, Callback, use_state, Properties};
use atoms::{InputText,InputSmallNumber,Heading2,HeadingDescription,ButtonLarge};
use layouting::{BodyItems,BottomOperaton};

#[derive(Clone)]
struct Item {
    name: String,
    count: usize,
}

#[derive(Properties,PartialEq)]
pub struct Props {
    pub room_id: String,
}

#[function_component(RuleMake)]
pub fn rule_make(props: &Props) -> Html {
    let state = use_state(|| vec![
        Item {
            name: "市民".to_string(),
            count: 3,
        },
        Item {
            name: "人狼".to_string(),
            count: 1,
        },
    ]);
    let captured_state = (*state).clone();
    let room_id = props.room_id.clone();
    let publish_rule = Callback::from(move |_| {
        firestore::set_document(
    &(),
    room_id.as_str(),
            &SetRule {
                rule:  Rule {
                    roles: captured_state
                        .iter()
                        .enumerate()
                        .map(|(index,item)| Role {name: item.name.clone(),number: item.count, id: index.to_string()}).collect()
                },
            },
            || {},
            || {},
        );
    });
    let captured_state = (*state).clone();
    let members = use_collection_sync::<MemberJSON>(&props.room_id);
    html! {
        <section class="mx-auto w-full max-w-2xl py-2">
                {
                    match members {
                        firestore_hooks::DataFetchState::Loading => Default::default(),
                        firestore_hooks::DataFetchState::Loaded(members) => html! {
                            <>
                                <BodyItems>
                                    <Heading2>{"ルールを決めましょう"}</Heading2>
                                    <HeadingDescription>{"役職とその人数を決めましょう"}</HeadingDescription>
                                    <HeadingDescription>{"現在 "}<b>{members.len()}</b>{"人のプレイヤーが参加中"}</HeadingDescription>
                                    <ul class="flex flex-col gap-2 mt-8">
                                        {for (*state).iter().enumerate().map(|(index,item)| {
                                            let on_number_input = {
                                                let mut captured_state = captured_state.clone();
                                                let state = state.clone();  
                                                Callback::once(move |count| {
                                                    captured_state[index].count = count;
                                                    state.set(captured_state)
                                                })
                                            };
                                            let on_text_input = {
                                                let mut captured_state = captured_state.clone();
                                                let state = state.clone();  
                                                Callback::once(move |name| {
                                                    captured_state[index].name = name;
                                                    state.set(captured_state)
                                                })
                                            };
                                            html! {
                                                <li class="flex justify-center gap-3 w-full">
                                                    <InputText
                                                        value={item.name.clone()}
                                                        placeholder="役職"
                                                        oninput={on_text_input}
                                                    />
                                                    <InputSmallNumber
                                                        value={item.count}
                                                        oninput={on_number_input}
                                                    />
                                                </li>
                                            }
                                        }
                                    )}
                                    </ul>
                                    <div class="flex justify-center mt-5">
                                        <button 
                                            onclick={Callback::from(move |_| {
                                                let mut captured_state = captured_state.clone();
                                                captured_state.push(Item {
                                                    name: "".to_string(),
                                                    count: 1,
                                                });
                                                state.set(captured_state)
                                            })} 
                                            class="text-black hover:text-black-light " 
                                        >
                                            <svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke-width="1.5" stroke="currentColor" class="w-6 h-6">
                                                <path stroke-linecap="round" stroke-linejoin="round" d="M12 4.5v15m7.5-7.5h-15" />
                                            </svg>
                                        </button>
                                    </div>    
                                </BodyItems>
                                <BottomOperaton>
                                        <ButtonLarge onclick={publish_rule}>
                                            {"ルールを確定"}
                                        </ButtonLarge>
                                </BottomOperaton>
                            </>
                        },
                    }
                }
        </section>
    }
}