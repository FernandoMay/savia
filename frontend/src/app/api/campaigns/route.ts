import { NextResponse } from "next/server";
import { db } from "@/lib/db";

export async function GET() {
  try {
    const campaigns = await db.campaign.findMany({
      where: { status: "ACTIVE" },
      include: { creator: true, transactions: true },
      orderBy: { createdAt: "desc" },
    });
    return NextResponse.json(campaigns);
  } catch {
    return NextResponse.json(
      { error: "Failed to fetch campaigns" },
      { status: 500 },
    );
  }
}

export async function POST(request: Request) {
  try {
    const body = await request.json();
    const campaign = await db.campaign.create({
      data: {
        title: body.title,
        description: body.description,
        goalAmount: body.goalAmount,
        category: body.category,
        location: body.location,
        medicalCondition: body.medicalCondition,
        beneficiaryAddress: body.beneficiaryAddress,
        creatorId: body.creatorId,
        endDate: new Date(body.endDate),
        imageUrl: body.imageUrl,
      },
    });
    return NextResponse.json(campaign, { status: 201 });
  } catch {
    return NextResponse.json(
      { error: "Failed to create campaign" },
      { status: 500 },
    );
  }
}
