import { useEffect, useState } from 'react';
import './App.css'
import ReactLogo from "./assets/react.svg"
import Todo from './components/Todo';

function App() {

  const [todos, setTodos] = useState([]);

  useEffect(() => {
    fetch("/api/tasks").then((response) => {
      response.json().then((json) => {
        setTodos(json);
      })
    })
  }, [])

  return (
    <div className="container">
      <h1 className="d-flex gap-2 align-items-center">
        <img src={ReactLogo} alt="" />
        <span>Todo List</span>
      </h1>
      {/* <BoxInput /> */}
      <hr />

      <TodoList todos={todos} />
    </div>
  )
}

function TodoList({ todos }) {

  const list = todos.map((x, i) => <Todo key={i} name={x.name} checked={x.complete} />)
  return (
    <ul>{list}</ul>
  )
}

export default App
