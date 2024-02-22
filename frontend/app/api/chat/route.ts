// import type { NextApiRequest, NextApiResponse } from 'next';

export async function POST(req: Request) {
  const json = await req.json();
  const { message } = json;
  const messageText = JSON.stringify(message);
  const payload = {
    text: messageText,
  };

  const apiResponse = await fetch("http://127.0.0.1:8080/api/chat", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(payload),
  });

  if (!apiResponse.ok) {
    const errorResponse = await apiResponse.text(); // ou `apiResponse.json()` dependendo do seu backend
    console.error("Erro na API:", errorResponse);
    throw new Error(`HTTP error! Status: ${apiResponse.status}`);
  }

  const data = await apiResponse.json();
  // console.log("Resposta:", data.choices[0].message.content);
  const completion = data.choices[0].message.content;
  // console.log('mensagem enviada route:', messageText)
  // console.log('resposta route:',completion)

  return new Response(completion, {
    headers: {
      "Content-Type": "text/plain",
    },
  });
}
