import NextAuth, { DefaultSession } from "next-auth"

export type ExtendeedUser = DefaultSession["user"] & {
  role: "ADMIN" | "USER",
  auth_token: string,
}

declare module "next-auth" {
  interface Session {
    user: ExtendeedUser;
  }
}
