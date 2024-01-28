import NextAuth, { type Session } from "next-auth"

import authConfig from "@/auth.config"
import { type JWT } from "next-auth/jwt";


export const {
	handlers: { GET, POST },
	auth,
	signIn,
	signOut,
} = NextAuth({
	callbacks: {
		// // TODO: verify email in backend 
		// async signIn({ user }) {
		// 	const existingUser = await getUserById(user.id);

		// 	if (!existingUser || !existingUser.emailVerified) {
		// 		return false;
		// 	}

		// 	return true;
		// },
		async jwt({ token, account, user }) {
			if (!token.sub) return token;
			if (!user && !account) return token;

			token["auth_token"] = user.auth_token;
			token["role"] = user.role;
			return token;
		},
		async session({ session, token }: { session: Session; token?: JWT }) {
			if (!token) return session;
			if (token.sub && token.role && session.user) {
				session.user.id = token.sub;
				session.user.role = token.role as "USER" | "ADMIN";
				session.user.auth_token = token.auth_token as string;
			}
			return session;
		},
	},
	session: {
		strategy: "jwt",
		maxAge: 60 * 60, // 1 hour
		updateAge: 30 * 60, // 30 minutes
	},
	jwt: {
		maxAge: 60 * 60, // 1 hour
	},
	...authConfig,
});
