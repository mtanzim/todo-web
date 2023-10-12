import { h, render } from "https://esm.sh/preact";
import { useEffect, useState } from "https://esm.sh/preact/hooks";
import htm from "https://esm.sh/htm";

const html = htm.bind(h);
render(html`<${App} />`, document.getElementById("app"));

function App() {
  const [tasks, setTasks] = useState([]);
  useEffect(() => {
    getTodos().then((v) => {
      setTasks(v);
      console.log({ tasks: v });
    });
  }, []);

  const onAdd = async (name) => {
    const isSuccess = await createTask(name);
    if (!isSuccess) {
      return;
    }
    const newTasks = await getTodos();
    setTasks(newTasks);
  };

  const onDelete = async (id) => {
    const isSuccess = await deleteTask(id);
    if (!isSuccess) {
      return;
    }
    const newTasks = await getTodos();
    setTasks(newTasks);
  };

  const onComplete = async (id) => {
    const isSuccess = await completeTask(id);
    if (!isSuccess) {
      return;
    }
    const newTasks = await getTodos();
    setTasks(newTasks);
  };

  return html`
    <div class="m-12">
      <h1 class="text-4xl">Todo App</h1>
      <div class="divider mb-12"></div>
      <h2 class="text-2xl my-4">Add a new task</h1>
      <${NewTask} onAdd=${onAdd} />
      <div class="divider my-12"></div>
      <h2 class="text-2xl my-4">Pending tasks</h1>
      <${List}
        tasks=${tasks.filter((t) => t.completed === 0)}
        onCTA=${onComplete}
      />
      <div class="divider my-12"></div>
      <h2 class="text-2xl my-4">Completed tasks</h1>
      <${List}
        tasks=${tasks.filter((t) => t.completed === 1)}
        onCTA=${onDelete}
      />
      <div></div>
    </div>
  `;
}

function NewTask({ onAdd }) {
  const [name, setName] = useState("");
  const submitHandler = (e) => {
    onAdd(name);
    setName("");
  };
  return html`
    <div class="form-control">
      <div class="input-group">
        <input
          type="text"
          placeholder="Task name"
          class="input input-bordered"
          value=${name}
          onChange=${(e) => setName(e.target.value)}
        />
        <button onClick=${submitHandler} class="btn btn-primary">
          Add Task
        </button>
      </div>
    </div>
  `;
}

function Task(props) {
  const { id, name, completed, onCTA } = props;
  return html`<div class="card w-96 bg-base-100 shadow-xl">
    <div class="card-body">
      <h2 class="card-title">${name}</h2>
      <div class="card-actions justify-end">
        <${CTA} completed=${completed} id=${id} onCTA=${onCTA} />
      </div>
    </div>
  </div>`;
}

function CTA({ completed, id, onCTA }) {
  if (completed) {
    return html`<button onClick=${() => onCTA(id)} class="btn btn-error">
      Delete
    </button>`;
  }
  return html`<button onClick=${() => onCTA(id)} class="btn btn-success">
    Complete
  </button>`;
}

function List(props) {
  const { tasks, onCTA } = props;

  return html`
    <div class="flex gap-8 flex-wrap">
      ${tasks.map((task) => html`<${Task} ...${task} onCTA=${onCTA} />`)}
    </div>
  `;
}

// API calls
async function getTodos() {
  const res = await fetch("/api/list");
  if (res.ok) {
    const todos = await res.json();
    return todos;
  }
  console.log("Error fetching todos");
  return [];
}
async function completeTask(id) {
  const res = await fetch(`/api/done/${id}`, {
    method: "PATCH",
  });
  if (res.ok) {
    const resText = await res.text();
    console.log({ resText });
    return true;
  }
  console.log("Error completing task");
  return false;
}
async function deleteTask(id) {
  const res = await fetch(`/api/destroy/${id}`, {
    method: "DELETE",
  });
  if (res.ok) {
    const resText = await res.text();
    console.log({ resText });
    return true;
  }
  console.log("Error deleting task");
  return false;
}
async function createTask(name) {
  if (name.length === 0) {
    console.log("Empty task name");
    return false;
  }
  const res = await fetch(`/api/add`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({ name }),
  });
  if (res.ok) {
    const resText = await res.text();
    console.log({ resText });
    return true;
  }
  console.log("Error creating task");
  return false;
}
