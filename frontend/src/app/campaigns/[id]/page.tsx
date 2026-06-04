"use client";

import { useState, useEffect } from "react";
import { useParams } from "next/navigation";
import {
  ArrowLeft,
  Heart,
  Shield,
  Clock,
  User,
  CheckCircle,
  XCircle,
} from "lucide-react";

interface CampaignDetail {
  id: string;
  title: string;
  description: string;
  goalAmount: number;
  currentAmount: number;
  category: string | null;
  location: string | null;
  medicalCondition: string | null;
  beneficiaryAddress: string;
  endDate: string;
  status: string;
  creator: { id: string; name: string | null; wallet: string };
  transactions: {
    id: string;
    amount: number;
    status: string;
    createdAt: string;
    donor: { wallet: string; name: string | null };
  }[];
}

export default function CampaignDetailPage() {
  const params = useParams();
  const [campaign, setCampaign] = useState<CampaignDetail | null>(null);
  const [amount, setAmount] = useState("");
  const [donating, setDonating] = useState(false);

  useEffect(() => {
    fetch(`/api/campaigns/${params.id}`)
      .then((r) => r.json())
      .then(setCampaign)
      .catch(() => {});
  }, [params.id]);

  if (!campaign) {
    return (
      <div className="flex min-h-screen items-center justify-center bg-slate-950">
        <p className="text-slate-500">Loading...</p>
      </div>
    );
  }

  const progress = Math.min(
    (campaign.currentAmount / campaign.goalAmount) * 100,
    100,
  );
  const daysLeft = Math.max(
    0,
    Math.ceil(
      (new Date(campaign.endDate).getTime() - Date.now()) / (1000 * 60 * 60 * 24),
    ),
  );
  const kycVerified = true;
  const trustScore = 85;

  const handleDonate = async () => {
    if (!amount) return;
    setDonating(true);
    try {
      await fetch("/api/transactions", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          campaignId: campaign.id,
          donorId: campaign.creator.id,
          amount: parseFloat(amount),
        }),
      });
      setAmount("");
      const res = await fetch(`/api/campaigns/${params.id}`);
      setCampaign(await res.json());
    } finally {
      setDonating(false);
    }
  };

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950">
      <div className="mx-auto max-w-4xl px-6 py-8">
        <a
          href="/"
          className="mb-6 inline-flex items-center gap-1 text-sm text-slate-400 hover:text-white"
        >
          <ArrowLeft className="h-4 w-4" /> Volver
        </a>

        <div className="mb-8">
          <div className="mb-2 flex items-center gap-2">
            <span className="rounded-full bg-green-500/10 px-3 py-1 text-xs text-green-400">
              {campaign.category || "General"}
            </span>
            {campaign.location && (
              <span className="text-xs text-slate-500">{campaign.location}</span>
            )}
            <span className="flex items-center gap-1 rounded-full bg-blue-500/10 px-3 py-1 text-xs text-blue-400">
              <Clock className="h-3 w-3" /> {daysLeft} days left
            </span>
          </div>
          <h1 className="mb-4 text-4xl font-bold text-white">
            {campaign.title}
          </h1>
          <p className="mb-6 text-lg text-slate-400">
            {campaign.description}
          </p>

          {campaign.medicalCondition && (
            <div className="mb-6 rounded-lg bg-slate-800/50 p-4">
              <p className="text-sm text-slate-400">Medical Condition</p>
              <p className="font-medium text-white">
                {campaign.medicalCondition}
              </p>
            </div>
          )}
        </div>

        <div className="mb-8 rounded-xl border border-slate-800 bg-slate-900/50 p-6">
          <div className="mb-4 flex items-center justify-between">
            <span className="text-3xl font-bold text-green-400">
              {campaign.currentAmount.toFixed(2)} XLM
            </span>
            <span className="text-slate-500">
              of {campaign.goalAmount.toFixed(2)} XLM
            </span>
          </div>
          <div className="mb-2 h-3 overflow-hidden rounded-full bg-slate-800">
            <div
              className="h-full rounded-full bg-gradient-to-r from-green-500 to-emerald-400 transition-all"
              style={{ width: `${progress}%` }}
            />
          </div>
          <p className="text-sm text-slate-500">{progress.toFixed(1)}% funded</p>
        </div>

        <div className="mb-8 grid gap-4 sm:grid-cols-2">
          <div className="rounded-xl border border-slate-800 bg-slate-900/50 p-4">
            <div className="flex items-center gap-2 text-sm text-slate-400">
              <Shield className="h-4 w-4" /> KYC Status
            </div>
            <p className="mt-1 flex items-center gap-1 font-medium text-green-400">
              <CheckCircle className="h-4 w-4" /> Verified
            </p>
          </div>
          <div className="rounded-xl border border-slate-800 bg-slate-900/50 p-4">
            <div className="flex items-center gap-2 text-sm text-slate-400">
              <Heart className="h-4 w-4" /> Trust Score
            </div>
            <p className="mt-1 font-medium text-white">{trustScore}/100</p>
          </div>
        </div>

        <div className="mb-8 rounded-xl border border-slate-800 bg-slate-900/50 p-6">
          <h3 className="mb-4 font-semibold text-white">Donate</h3>
          <div className="flex gap-3">
            <input
              type="number"
              step="0.01"
              min="0"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              placeholder="Amount in XLM"
              className="flex-1 rounded-lg border border-slate-700 bg-slate-800 px-4 py-2 text-white placeholder-slate-500 focus:border-green-500 focus:outline-none"
            />
            <button
              onClick={handleDonate}
              disabled={donating || !amount}
              className="rounded-lg bg-green-600 px-6 py-2 font-medium text-white hover:bg-green-700 disabled:opacity-50"
            >
              {donating ? "Processing..." : "Donate"}
            </button>
          </div>
        </div>

        <div>
          <h3 className="mb-4 text-lg font-semibold text-white">
            Recent Transactions
          </h3>
          <div className="overflow-x-auto rounded-xl border border-slate-800">
            <table className="w-full text-sm">
              <thead>
                <tr className="border-b border-slate-800 bg-slate-900/50">
                  <th className="px-4 py-3 text-left text-slate-400">Donor</th>
                  <th className="px-4 py-3 text-right text-slate-400">
                    Amount
                  </th>
                  <th className="px-4 py-3 text-right text-slate-400">Date</th>
                  <th className="px-4 py-3 text-right text-slate-400">
                    Status
                  </th>
                </tr>
              </thead>
              <tbody>
                {campaign.transactions.map((tx) => (
                  <tr key={tx.id} className="border-b border-slate-800/50">
                    <td className="px-4 py-3 text-white">
                      {tx.donor.name || tx.donor.wallet.slice(0, 8)}...
                    </td>
                    <td className="px-4 py-3 text-right text-green-400">
                      {tx.amount.toFixed(2)} XLM
                    </td>
                    <td className="px-4 py-3 text-right text-slate-500">
                      {new Date(tx.createdAt).toLocaleDateString()}
                    </td>
                    <td className="px-4 py-3 text-right">
                      <span className="rounded-full bg-green-500/10 px-2 py-0.5 text-xs text-green-400">
                        {tx.status}
                      </span>
                    </td>
                  </tr>
                ))}
                {campaign.transactions.length === 0 && (
                  <tr>
                    <td
                      colSpan={4}
                      className="px-4 py-8 text-center text-slate-500"
                    >
                      No transactions yet
                    </td>
                  </tr>
                )}
              </tbody>
            </table>
          </div>
        </div>
      </div>
    </div>
  );
}
