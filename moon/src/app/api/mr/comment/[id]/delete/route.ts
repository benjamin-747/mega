import { verifySession } from "@/app/lib/dal";

export async function POST(request: Request, { params }: { params: { id: string } }) {
    const session = await verifySession()

    // Check if the user is authenticated
    if (!session) {
        // User is not authenticated
        return new Response(null, { status: 401 })
    }

    const endpoint = process.env.MEGA_INTERNAL_HOST;
    const res = await fetch(`${endpoint}/api/v1/mr/comment/${params.id}/delete`, {
        method: 'POST',
    })
    const data = await res.json()

    return Response.json({ data })
}