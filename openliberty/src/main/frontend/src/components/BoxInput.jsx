export default function BoxInput() {

  const keydown = (ev) => {
    if (ev.code == "Enter") go()
  }

  const go = () => {

  }

  return (
      <div className="input-group">
        <input onKeyDown={keydown} className="form-control" type="text" name="task" placeholder='Input new task' id="" />
        <button onClick={go} className="btn btn-primary">+</button>
      </div>
  )
}