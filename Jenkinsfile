pipeline {
    agent any
    stages {
        stage('Clone repository') {
            steps {
                checkout scm
            }
        }
    
        stage('Build image') {
            steps {
                script {
                    app = docker.build("iceh2/lemmyremindyou")
                }
            }
        }
    
        stage('Push image') {
            when {
                tag 'v*.*.*'
            }
            steps {
                script {
                    docker.withRegistry('https://registry.hub.docker.com', 'docker-hub-credentials') {
                        app.push("${env.TAG_NAME}")
                        app.push("latest")
                    }
                }
            }
        }
    }
}