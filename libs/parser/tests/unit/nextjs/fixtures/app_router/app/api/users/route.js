export async function GET() {
  return Response.json({ users: [] });
}

export async function POST(request) {
  const body = await request.json();
  return Response.json({ id: 1, ...body });
}
