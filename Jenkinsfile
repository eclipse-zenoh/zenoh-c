pipeline {
  agent { label 'UbuntuVM' }
  parameters {
    gitParameter name: 'TAG', 
                 type: 'PT_TAG',
                 defaultValue: 'master'
  }

  stages {
    stage('Checkout Git TAG') {
      steps {
        cleanWs()
        checkout([$class: 'GitSCM',
                  branches: [[name: "${params.TAG}"]],
                  doGenerateSubmoduleConfigurations: false,
                  extensions: [],
                  gitTool: 'Default',
                  submoduleCfg: [],
                  userRemoteConfigs: [[url: 'https://github.com/eclipse-zenoh/zenoh-c.git']]
                ])
      }
    }
    stage('Simple build') {
      steps {
        sh '''
        git log --graph --date=short --pretty=tformat:'%ad - %h - %cn -%d %s' -n 20 || true
        make all
        '''
      }
    }
    stage('Cross-platforms build') {
      steps {
        sh '''
        docker images || true
        make all-cross
        '''
      }
    }
    stage('Deploy to to download.eclipse.org') {
      steps {
        sshagent ( ['projects-storage.eclipse.org-bot-ssh']) {
          sh '''
          ssh genie.zenoh@projects-storage.eclipse.org mkdir -p /home/data/httpd/download.eclipse.org/zenoh/zenoh-c/${TAG}
          scp build/crossbuilds/*/*.deb* build/crossbuilds/*/*.rpm*  genie.zenoh@projects-storage.eclipse.org:/home/data/httpd/download.eclipse.org/zenoh/zenoh-c/${TAG}/
          '''
        }
      }
    }
  }

  post {
    success {
        archiveArtifacts artifacts: 'build/crossbuilds/*/*zenohc.*', fingerprint: true
    }
  }
}
