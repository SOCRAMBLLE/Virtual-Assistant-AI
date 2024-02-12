// import type { NextApiRequest, NextApiResponse } from 'next';

export async function POST(req: Request) {
  const json = await req.json();
  const { message } = json;
  const messageText = JSON.stringify(message)
  const payload = {
    role: "user",
    content: messageText,
  };
  
    const apiResponse = await fetch("https://backend-virtual-ai.onrender.com/api/chat", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  });

  if (!apiResponse.ok) {
    return new Response('HTTP error! Status:', { status: apiResponse.status } );
  }

  const data = await apiResponse.json();
  const completion = data.data.content.trim()
  // console.log('mensagem enviada route:', messageText)
  // console.log('resposta route:',completion)
  
  return new Response(completion, {
    headers: {
      'Content-Type': 'text/plain'
    }
  })
}
