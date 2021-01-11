pipeline {
  agent {
    docker {
      image 'scssubstratee/substratee_dev:18.04-2.9.1-1.1.2'
      args '''
        -u root
        --privileged
      '''
    }
  }
  options {
    timeout(time: 1, unit: 'HOURS')
    buildDiscarder(logRotator(numToKeepStr: '3'))
  }
  stages {
    stage('Update') {
      steps {
        sh './ci/install_wasm.sh'
      }
    }
    stage('Build') {
      steps {
        sh 'cargo build --release'
      }
    }
    stage('Archive artifact') {
      steps {
        archiveArtifacts artifacts: 'target/release/node-template', fingerprint: true
      }
    }
  }
  post {
    unsuccessful {
        emailext (
          subject: "Jenkins Build '${env.JOB_NAME} [${env.BUILD_NUMBER}]' is ${currentBuild.currentResult}",
          body: "${env.JOB_NAME} build ${env.BUILD_NUMBER} changed state and is now ${currentBuild.currentResult}\n\nMore info at: ${env.BUILD_URL}",
          to: '${env.RECIPIENTS_SUBSTRATEE}'
        )
    }
    always {
      cleanWs()
    }
  }
}