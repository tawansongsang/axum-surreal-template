import type { NextAuthConfig } from "next-auth"
import Credentials from "next-auth/providers/credentials"

import { LoginSchema } from "@/schemas";

export default {
	providers: [
		Credentials({
			async authorize(credentials) {
				// console.log("auth.config.ts");
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
					// TODO: setting addtional cookies to client
					// console.log(cookies);
					const backend_cookies = cookies[0].split(";")[0];
					// console.log(backend_cookies);
					const response_json = await response.json();
					// console.log(response_json);
					const user = response_json.data;
					user["auth_token"] = backend_cookies;
					// console.log(user);
					if (status === 200) return user;
				}

				return null;
			}
		})
	],
} satisfies NextAuthConfig
