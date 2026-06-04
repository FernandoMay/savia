# Savia — Crowdfunding con Propósito

Plataforma de crowdfunding solidario construida sobre Stellar Soroban.

## Stack

- **Frontend:** Next.js 14, React 18, Tailwind CSS, shadcn/ui
- **Backend:** Next.js API Routes + Prisma ORM
- **Database:** PostgreSQL
- **Blockchain:** Stellar Soroban (via Freighter wallet)

## Setup

```bash
# Install dependencies
npm install

# Generate Prisma client
npm run db:generate

# Push schema to database
npm run db:push

# Start dev server
npm run dev
```

## Prisma Studio

```bash
npm run db:studio
```

## Environment Variables

Create a `.env` file:

```
DATABASE_URL="postgresql://user:password@localhost:5432/savia"
```

## Deploy on Vercel

1. Push to GitHub
2. Import project in Vercel
3. Set `DATABASE_URL` environment variable
4. Deploy
