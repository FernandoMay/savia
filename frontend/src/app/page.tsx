"use client";

import { useState, useEffect } from "react";
import { Plus, Wallet, ArrowRight, Heart } from "lucide-react";

interface Campaign {
  id: string;
  title: string;
  description: string;
  goalAmount: number;
  currentAmount: number;
  category: string | null;
  location: string | null;
  status: string;
  imageUrl: string | null;
}

export default function HomePage() {
  const [campaigns, setCampaigns] = useState<Campaign[]>([]);
  const [walletConnected, setWalletConnected] = useState(false);

  useEffect(() => {
    fetch("/api/campaigns")
      .then((r) => r.json())
      .then(setCampaigns)
      .catch(() => {});
  }, []);

  return (
    <div className="min-h-screen bg-gradient-to-b from-slate-950 via-slate-900 to-slate-950">
      <header className="border-b border-slate-800 px-6 py-4">
        <div className="mx-auto flex max-w-7xl items-center justify-between">
          <div className="flex items-center gap-2">
            <Heart className="h-6 w-6 text-green-500" />
            <span className="text-xl font-bold text-white">Savia</span>
          </div>
          <div className="flex items-center gap-4">
            <a
              href="/create"
              className="flex items-center gap-2 rounded-lg bg-green-600 px-4 py-2 text-sm font-medium text-white hover:bg-green-700"
            >
              <Plus className="h-4 w-4" />
              Crear Campaña
            </a>
            <button
              onClick={() => setWalletConnected(!walletConnected)}
              className="flex items-center gap-2 rounded-lg border border-slate-700 px-4 py-2 text-sm text-slate-300 hover:bg-slate-800"
            >
              <Wallet className="h-4 w-4" />
              {walletConnected ? "0x...a1b2" : "Conectar Wallet"}
            </button>
          </div>
        </div>
      </header>

      <main className="mx-auto max-w-7xl px-6 py-16">
        <section className="mb-20 text-center">
          <h1 className="mb-4 bg-gradient-to-r from-green-400 to-emerald-300 bg-clip-text text-5xl font-bold text-transparent">
            Crowdfunding con Propósito
          </h1>
          <p className="mx-auto mb-8 max-w-2xl text-lg text-slate-400">
            Apoya causas médicas con transparencia total gracias a la blockchain
            Soroban. Cada donación es trazable, segura y eficiente.
          </p>
          <div className="flex justify-center gap-4">
            <a
              href="/create"
              className="rounded-lg bg-green-600 px-6 py-3 font-medium text-white hover:bg-green-700"
            >
              Crear Campaña
            </a>
            <a
              href="#campaigns"
              className="rounded-lg border border-slate-700 px-6 py-3 font-medium text-slate-300 hover:bg-slate-800"
            >
              Explorar Causas
            </a>
          </div>
        </section>

        <section id="campaigns">
          <h2 className="mb-8 text-2xl font-bold text-white">
            Campañas Activas
          </h2>
          <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
            {campaigns.map((campaign) => {
              const progress = Math.min(
                (campaign.currentAmount / campaign.goalAmount) * 100,
                100,
              );
              return (
                <a
                  key={campaign.id}
                  href={`/campaigns/${campaign.id}`}
                  className="group rounded-xl border border-slate-800 bg-slate-900/50 p-6 transition hover:border-green-500/50 hover:bg-slate-800/50"
                >
                  {campaign.imageUrl && (
                    <img
                      src={campaign.imageUrl}
                      alt={campaign.title}
                      className="mb-4 h-40 w-full rounded-lg object-cover"
                    />
                  )}
                  <div className="mb-2 flex items-center gap-2">
                    <span className="rounded-full bg-green-500/10 px-2 py-0.5 text-xs text-green-400">
                      {campaign.category || "General"}
                    </span>
                    {campaign.location && (
                      <span className="text-xs text-slate-500">
                        {campaign.location}
                      </span>
                    )}
                  </div>
                  <h3 className="mb-2 text-lg font-semibold text-white group-hover:text-green-400">
                    {campaign.title}
                  </h3>
                  <p className="mb-4 line-clamp-2 text-sm text-slate-400">
                    {campaign.description}
                  </p>
                  <div className="mb-2 h-2 overflow-hidden rounded-full bg-slate-800">
                    <div
                      className="h-full rounded-full bg-gradient-to-r from-green-500 to-emerald-400 transition-all"
                      style={{ width: `${progress}%` }}
                    />
                  </div>
                  <div className="flex justify-between text-sm">
                    <span className="text-green-400">
                      {campaign.currentAmount.toFixed(2)} XLM
                    </span>
                    <span className="text-slate-500">
                      meta {campaign.goalAmount.toFixed(2)} XLM
                    </span>
                  </div>
                </a>
              );
            })}
            {campaigns.length === 0 && (
              <div className="col-span-full rounded-xl border border-dashed border-slate-800 p-12 text-center text-slate-500">
                <Heart className="mx-auto mb-4 h-8 w-8" />
                <p className="mb-2 text-lg">No hay campañas activas aún</p>
                <a
                  href="/create"
                  className="inline-flex items-center gap-1 text-green-400 hover:text-green-300"
                >
                  Sé el primero en crear una <ArrowRight className="h-4 w-4" />
                </a>
              </div>
            )}
          </div>
        </section>
      </main>
    </div>
  );
}
