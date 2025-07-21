from fastapi import FastAPI
from pydantic import BaseModel

app = FastAPI()

class PredictionRequest(BaseModel):
    data: dict

@app.post('/predict')
async def predict(request: PredictionRequest):
    # Mock ML prediction
    return {"prediction": "positive", "confidence": 0.95}

@app.get('/health')
async def health():
    return {"status": "healthy"}
