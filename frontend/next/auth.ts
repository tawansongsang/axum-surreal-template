import NextAuth from "next-auth"

import authConfig from "@/auth.config"

export const {
	handlers: { GET, POST },
	auth,
	signIn,
	signOut,
} = NextAuth({
	callbacks: {
		async jwt({ token, account, user }) {
			if (!token.sub) return token;
			if (!user && !account) return token;

			console.log({ Account: account });
			console.log({ User: user });
			token["auth_token"] = user.auth_token;
			token["role"] = user.role;
			// TODO: adding cookies backend to token
			console.log({ Token: token });
			// token["auth_token"] = user["auth_token"];
			return token;
		},
		async session({ session, token }) {
			// console.log(session);
			console.log({ TokenSession: token });
			if (token.sub && session.user) {
				session.user.id = token.sub;
				session.user.role = token.role;
			}
			console.log({ sessionToken: session });
			return session;
		},
	},
	session: { strategy: "jwt" },
	...authConfig,
});
