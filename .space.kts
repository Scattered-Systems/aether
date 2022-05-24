job("Build and Push: aether") {
    docker {
        build {
            context = "./aether"
            file = "./aether/Dockerfile"
            args["HTTP_PROXY"] = "http://0.0.0.0:8888"
            labels["vendor"] = "scattered-systems"
        }

        push("scattered-systems.registry.jetbrains.space/p/scsys/containers/aether") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER", "latest")
        }
    }
}