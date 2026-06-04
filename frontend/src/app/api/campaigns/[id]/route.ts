import { NextRequest, NextResponse } from "next/server";
import { db } from "@/lib/db";

export async function GET(
  _request: NextRequest,
  { params }: { params: { id: string } },
) {
  try {
    const campaign = await db.campaign.findUnique({
      where: { id: params.id },
      include: {
        creator: true,
        transactions: {
          include: { donor: true },
          orderBy: { createdAt: "desc" },
        },
      },
    });
    if (!campaign) {
      return NextResponse.json(
        { error: "Campaign not found" },
        { status: 404 },
      );
    }
    return NextResponse.json(campaign);
  } catch {
    return NextResponse.json(
      { error: "Failed to fetch campaign" },
      { status: 500 },
    );
  }
}
