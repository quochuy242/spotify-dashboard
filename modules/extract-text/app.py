"""
Flask API service for extract-text module
Provides endpoints for tele-bot module to send user input and receive extracted metadata
"""

from flask import Flask, request, jsonify
from typing import Dict, Any
from extract import extract_text, MusicMetadata

app = Flask(__name__)


@app.route('/health', methods=['GET'])
def health_check():
    """Health check endpoint"""
    return jsonify({"status": "ok", "service": "extract-text"}), 200


@app.route('/extract', methods=['POST'])
def extract_endpoint():
    """
    Extract music metadata from user input text
    
    Expected JSON:
    {
        "text": "user input text",
        "user_id": "optional user identifier"
    }
    
    Returns:
    {
        "success": true/false,
        "data": {
            "track": "...",
            "artist": "...",
            ...
        },
        "message": "error message if any"
    }
    """
    try:
        data = request.get_json()
        
        if not data or 'text' not in data:
            return jsonify({
                "success": False,
                "message": "Missing 'text' field in request"
            }), 400
        
        user_input = data.get('text', '').strip()
        user_id = data.get('user_id')
        
        if not user_input:
            return jsonify({
                "success": False,
                "message": "Empty text provided"
            }), 400
        
        # Extract metadata
        metadata = extract_text(user_input)
        
        # Convert to dictionary with only non-None fields
        result = {k: v for k, v in metadata.dict().items() if v is not None}
        
        return jsonify({
            "success": True,
            "data": result,
            "user_id": user_id
        }), 200
        
    except Exception as e:
        return jsonify({
            "success": False,
            "message": f"Extraction failed: {str(e)}"
        }), 500


@app.route('/extract/batch', methods=['POST'])
def extract_batch_endpoint():
    """
    Extract metadata from multiple texts
    
    Expected JSON:
    {
        "texts": [
            {"text": "...", "user_id": "optional"},
            ...
        ]
    }
    
    Returns array of extraction results
    """
    try:
        data = request.get_json()
        
        if not data or 'texts' not in data:
            return jsonify({
                "success": False,
                "message": "Missing 'texts' field in request"
            }), 400
        
        texts = data.get('texts', [])
        
        if not isinstance(texts, list):
            return jsonify({
                "success": False,
                "message": "'texts' must be an array"
            }), 400
        
        results = []
        for item in texts:
            user_input = item.get('text', '').strip()
            user_id = item.get('user_id')
            
            if user_input:
                metadata = extract_text(user_input)
                result = {k: v for k, v in metadata.dict().items() if v is not None}
            else:
                result = {}
            
            results.append({
                "user_id": user_id,
                "data": result
            })
        
        return jsonify({
            "success": True,
            "results": results
        }), 200
        
    except Exception as e:
        return jsonify({
            "success": False,
            "message": f"Batch extraction failed: {str(e)}"
        }), 500


@app.errorhandler(404)
def not_found(error):
    return jsonify({
        "success": False,
        "message": "Endpoint not found"
    }), 404


@app.errorhandler(500)
def internal_error(error):
    return jsonify({
        "success": False,
        "message": "Internal server error"
    }), 500


if __name__ == '__main__':
    app.run(host='0.0.0.0', port=5001, debug=True)
