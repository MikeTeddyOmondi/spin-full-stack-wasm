import { axiosClient } from "./axios";

const getTodos = async function () {
  let response = await axiosClient.get("/api/todos");
  console.log({ response });
  return response;
};

export { getTodos };
