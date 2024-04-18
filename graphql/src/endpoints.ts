export const hello = () => {
  return "Hi!"
}

export const greeting = (params: ({ name: string })) => {
  return "Hello, " + params.name;
}