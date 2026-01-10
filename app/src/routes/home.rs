use crate::api::*;
use leptos::{prelude::*, task::spawn_local};
use singlestage::*;

#[component]
pub fn Home() -> impl IntoView {
    let add_todo = ServerAction::<AddTodo>::new();
    let pending = add_todo.pending();

    let refetch = RwSignal::new(false);
    let todos = Resource::new(move || refetch.get(), |_| get_todos());

    let form_ref = NodeRef::<leptos::html::Form>::new();

    Effect::new(move || {
        if !pending.get() {
            refetch.set(!refetch.get_untracked())
        }
    });

    view! {
        <main class="mx-4 w-sm space-y-4">
            <ActionForm
                node_ref=form_ref
                action=add_todo
                on:submit=move |_| {
                    form_ref.get_untracked().unwrap().reset();
                }
            >
                <InputGroup>
                    <Input name="title" placeholder="Add todo..." />
                    <InputGroupAddon align="inline-end">
                        <Button
                            aria_label="Add todo"
                            title="Add todo"
                            variant="secondary"
                            button_type="submit"
                        >
                            "Add"
                        </Button>
                    </InputGroupAddon>
                </InputGroup>
            </ActionForm>

            <Separator />

            <Transition fallback=move || {
                view! { <Spinner /> }
            }>
                {move || Suspend::new(async move {
                    let todos = todos.await.unwrap_or_default();
                    view! {
                        <ul>
                            <For
                                each=move || todos.to_owned()
                                key=|todo| todo.id.clone().unwrap()
                                let(todo)
                            >
                                <li>
                                    <Button
                                        aria_label="Delete todo"
                                        title="Delete todo"
                                        size="sm"
                                        variant="ghost"
                                        on:click=move |_| {
                                            let id = todo.id.clone().unwrap();
                                            spawn_local(async move {
                                                let res = delete_todo(id).await;
                                                if res.is_ok() {
                                                    refetch.set(!refetch.get_untracked())
                                                }
                                            });
                                        }
                                    >
                                        {todo.title}
                                    </Button>
                                </li>
                            </For>
                        </ul>
                    }
                })}
            </Transition>
        </main>
    }
}
