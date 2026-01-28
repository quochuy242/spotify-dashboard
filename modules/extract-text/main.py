"""Main entry point for extract-text module"""

from app import app


def main():
    """Start the extract-text service"""
    print("Starting extract-text service...")
    app.run(host='0.0.0.0', port=5001, debug=False)


if __name__ == "__main__":
    main()
