import { NextResponse } from "next/server";
import { db } from "@/lib/db";

export async function POST(request: Request) {
  try {
    const body = await request.json();
    let user = await db.user.findUnique({ where: { wallet: body.wallet } });
    if (!user) {
      user = await db.user.create({
        data: {
          wallet: body.wallet,
          name: body.name,
          email: body.email,
        },
      });
    }
    return NextResponse.json(user);
  } catch {
    return NextResponse.json(
      { error: "Authentication failed" },
      { status: 500 },
    );
  }
}
