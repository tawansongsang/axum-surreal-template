import type { NextAuthConfig } from "next-auth";
import Credentials from "next-auth/providers/credentials";
import Github from "next-auth/providers/github";
import Google from "next-auth/providers/google";

import { LoginSchema } from "@/schemas";

export default {
	providers: [
		Google({ // TODO: add user to backend
			clientId: process.env.GOOGLE_CLIENT_ID,
			clientSecret: process.env.GOOGLE_CLIENT_SECRETE,
		}),
		Github({
			clientId: process.env.GITHUB_CLIENT_ID,
			clientSecret: process.env.GITHUB_CLIENT_SECRET,
		}),
		Credentials({
			async authorize(credentials) {
				const validatedFields = LoginSchema.safeParse(credentials);

				if (validatedFields.success) {
					const { email, password } = validatedFields.data;
					const body = { username: email, password: password };
					const response = await fetch(`${process.env.BACKEND_URL}/login`, {
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
					user.auth_token = backend_cookies;
					if (status === 200) return user;
				}

				return null;
			}
		})
	],
} satisfies NextAuthConfig
