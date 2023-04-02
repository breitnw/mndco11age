from flask import Flask, abort, current_app, render_template, request

def create_app():
    app = Flask(__name__)
    
    @app.route('/')
    def index():
        if 'mndco11age.xyz' in request.base_url:
            return render_template('index.html');
        else:
            abort(404)

    @app.route('/empty')
    def empty():
        return render_template('empty.html');

    @app.route('/demo')
    def demo():
        return render_template('demo.html');

    return app
