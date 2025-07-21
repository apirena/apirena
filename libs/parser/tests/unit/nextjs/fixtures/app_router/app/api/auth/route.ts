export async function POST(request: Request) {
  const body = await request.json();
  
  // Mock auth logic
  return Response.json({ 
    token: 'mock-token',
    user: body.email 
  });
}
