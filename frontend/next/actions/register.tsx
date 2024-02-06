"use server";

import * as z from "zod";

import { RegisterSchema } from "@/schemas";

export const register = async (values: z.infer<typeof RegisterSchema>) => {
  // console.log(values);
  const validatedFields = RegisterSchema.safeParse(values);

  if (!validatedFields.success) {
    return { error: "Invalid fields!" };
  }

  const { email, password, name } = validatedFields.data;
  const response = await fetch(`${process.env.BACKEND_URL}/api/register`, {
    method: "POST",
    cache: "no-cache",
    body: JSON.stringify({
      "username": email,
      "name": name,
      "password": password,
    }),
    headers: {
      "Content-type": "application/json; charset=UTF-8",
    },
  });
  const status = response.status;

  if (status === 400) {
    let body = await response.json();
    // console.log(body);
    return { error: body.error.message };
  }

  // TODO: Send verification token email


  return { success: "User created!" };
};
