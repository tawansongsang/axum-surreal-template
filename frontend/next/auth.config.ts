import type { NextAuthConfig } from "next-auth"
import Credentials from "next-auth/providers/credentials"

import { LoginSchema } from "@/schemas";

export default {
	providers: [
		Credentials({
			async authorize(credentials) {
				const validatedFields = LoginSchema.safeParse(credentials);

				if (validatedFields.success) {
					const { email, password } = validatedFields.data;
					const body = { username: email, password: password };
					const response = await fetch("http://localhost:8080/api/login", {
						method: "POST",
						cache: "no-cache",
						headers: {
							"Content-Type": "application/json; charset=UTF-8"
						},
						body: JSON.stringify(body)
					})

					const status = response.status;
					const cookies = response.headers.getSetCookie();
					const backend_cookies = cookies[0].split(";")[0];
					const response_json = await response.json();
					const user = response_json.data;
					user["auth_token"] = backend_cookies;
					if (status === 200) return user;
				}

				return null;
			}
		})
	],
} satisfies NextAuthConfig
