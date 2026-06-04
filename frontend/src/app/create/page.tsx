"use client";

import { useState } from "react";
import { useRouter } from "next/navigation";
import { ArrowLeft, Heart } from "lucide-react";

export default function CreateCampaignPage() {
  const router = useRouter();
  const [form, setForm] = useState({
    title: "",
    description: "",
    goalAmount: "",
    category: "",
    location: "",
    medicalCondition: "",
    beneficiaryAddress: "",
    endDate: "",
  });
  const [submitting, setSubmitting] = useState(false);

  const handleSubmit = async (e: React.FormEvent) => {
    e.preventDefault();
    setSubmitting(true);
    try {
      const res = await fetch("/api/campaigns", {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({
          ...form,
          goalAmount: parseFloat(form.goalAmount),
          creatorId: "cm0000000000000000000001",
          endDate: new Date(form.endDate).toISOString(),
        }),
      });
      if (res.ok) {
        const campaign = await res.json();
        router.push(`/campaigns/${campaign.id}`);
      }
    } finally {
      setSubmitting(false);
    }
  };

  const update = (field: string) => (e: React.ChangeEvent<HTMLInputElement | HTMLTextAreaElement>) =>
    setForm({ ...form, [field]: e.target.value });

  const inputClass =
    "w-full rounded-lg border border-slate-700 bg-slate-800 px-4 py-2 text-white placeholder-slate-500 focus:border-green-500 focus:outline-none";
  const labelClass = "mb-1 block text-sm font-medium text-slate-300";

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950">
      <div className="mx-auto max-w-2xl px-6 py-8">
        <button
          onClick={() => router.back()}
          className="mb-6 inline-flex items-center gap-1 text-sm text-slate-400 hover:text-white"
        >
          <ArrowLeft className="h-4 w-4" /> Back
        </button>

        <div className="mb-8 flex items-center gap-2">
          <Heart className="h-6 w-6 text-green-500" />
          <h1 className="text-2xl font-bold text-white">Create Campaign</h1>
        </div>

        <form onSubmit={handleSubmit} className="space-y-6">
          <div>
            <label className={labelClass}>Title</label>
            <input
              type="text"
              value={form.title}
              onChange={update("title")}
              required
              className={inputClass}
              placeholder="Help Maria with cancer treatment"
            />
          </div>

          <div>
            <label className={labelClass}>Description</label>
            <textarea
              value={form.description}
              onChange={update("description")}
              required
              rows={4}
              className={inputClass}
              placeholder="Describe the campaign purpose..."
            />
          </div>

          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <label className={labelClass}>Goal Amount (XLM)</label>
              <input
                type="number"
                step="0.01"
                min="0"
                value={form.goalAmount}
                onChange={update("goalAmount")}
                required
                className={inputClass}
                placeholder="1000"
              />
            </div>
            <div>
              <label className={labelClass}>Category</label>
              <input
                type="text"
                value={form.category}
                onChange={update("category")}
                className={inputClass}
                placeholder="Medical, Education, etc."
              />
            </div>
          </div>

          <div className="grid gap-4 sm:grid-cols-2">
            <div>
              <label className={labelClass}>Location</label>
              <input
                type="text"
                value={form.location}
                onChange={update("location")}
                className={inputClass}
                placeholder="Mexico City"
              />
            </div>
            <div>
              <label className={labelClass}>Medical Condition</label>
              <input
                type="text"
                value={form.medicalCondition}
                onChange={update("medicalCondition")}
                className={inputClass}
                placeholder="Cancer treatment"
              />
            </div>
          </div>

          <div>
            <label className={labelClass}>Beneficiary Wallet Address</label>
            <input
              type="text"
              value={form.beneficiaryAddress}
              onChange={update("beneficiaryAddress")}
              required
              className={inputClass}
              placeholder="G... or 0x..."
            />
          </div>

          <div>
            <label className={labelClass}>End Date</label>
            <input
              type="date"
              value={form.endDate}
              onChange={update("endDate")}
              required
              className={inputClass}
            />
          </div>

          <button
            type="submit"
            disabled={submitting}
            className="w-full rounded-lg bg-green-600 py-3 font-medium text-white hover:bg-green-700 disabled:opacity-50"
          >
            {submitting ? "Creating..." : "Create Campaign"}
          </button>
        </form>
      </div>
    </div>
  );
}
