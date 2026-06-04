import { db } from "./db";

export interface CampaignData {
  title: string;
  description: string;
  goalAmount: number;
  category?: string;
  location?: string;
  medicalCondition?: string;
  beneficiaryAddress: string;
  creatorId: string;
  endDate: Date;
}

export interface DonationData {
  campaignId: string;
  donorId: string;
  amount: number;
}

export const stellar = {
  async getCampaigns() {
    return db.campaign.findMany({
      where: { status: "ACTIVE" },
      include: { creator: true, transactions: true },
      orderBy: { createdAt: "desc" },
    });
  },

  async getCampaign(id: string) {
    return db.campaign.findUnique({
      where: { id },
      include: {
        creator: true,
        transactions: { include: { donor: true }, orderBy: { createdAt: "desc" } },
      },
    });
  },

  async createCampaign(data: CampaignData) {
    return db.campaign.create({
      data: {
        title: data.title,
        description: data.description,
        goalAmount: data.goalAmount,
        category: data.category,
        location: data.location,
        medicalCondition: data.medicalCondition,
        beneficiaryAddress: data.beneficiaryAddress,
        creatorId: data.creatorId,
        endDate: data.endDate,
      },
    });
  },

  async donate(data: DonationData) {
    const [transaction] = await db.$transaction([
      db.transaction.create({
        data: {
          campaignId: data.campaignId,
          donorId: data.donorId,
          amount: data.amount,
          status: "CONFIRMED",
        },
      }),
      db.campaign.update({
        where: { id: data.campaignId },
        data: { currentAmount: { increment: data.amount } },
      }),
    ]);
    return transaction;
  },
};
