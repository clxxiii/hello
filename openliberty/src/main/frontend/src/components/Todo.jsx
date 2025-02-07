import { useState } from "react"

export default function Todo({name, checked}) {

  const [completed, setCompleted] = useState(checked);

  const toggle = () => {
    console.log(completed)
    setCompleted(!completed);
  }

  return (
    <div className="d-flex gap-2">
      <input className="form-check-input" type="checkbox" checked={completed} name="complete" onClick={toggle} />
      <span style={{ textDecoration: completed ? 'line-through' : 'none'}}>
        {name}
      </span>
    </div>
  )
}