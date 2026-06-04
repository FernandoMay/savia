import { NextResponse } from "next/server";
import { db } from "@/lib/db";

export async function GET() {
  try {
    const transactions = await db.transaction.findMany({
      include: { campaign: true, donor: true },
      orderBy: { createdAt: "desc" },
      take: 50,
    });
    return NextResponse.json(transactions);
  } catch {
    return NextResponse.json(
      { error: "Failed to fetch transactions" },
      { status: 500 },
    );
  }
}

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const [transaction] = await db.$transaction([
      db.transaction.create({
        data: {
          campaignId: body.campaignId,
          donorId: body.donorId,
          amount: body.amount,
          status: "CONFIRMED",
        },
      }),
      db.campaign.update({
        where: { id: body.campaignId },
        data: { currentAmount: { increment: body.amount } },
      }),
    ]);
    return NextResponse.json(transaction, { status: 201 });
  } catch {
    return NextResponse.json(
      { error: "Failed to create transaction" },
      { status: 500 },
    );
  }
}
