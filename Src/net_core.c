#include <sys/socket.h>
#include <netinet/in.h>
#include <arpa/inet.h>
#include <unistd.h>

int32_t c_scan_port(const char* ip, int32_t port) {
    int sockfd = socket(AF_INET, SOCK_STREAM, 0);
    if (sockfd < 0) return 0;

    struct sockaddr_in addr;
    addr.sin_family = AF_INET;
    addr.sin_port = htons(port);
    addr.sin_addr.s_addr = inet_addr(ip);

    // Attempt connection
    int result = connect(sockfd, (struct sockaddr*)&addr, sizeof(addr));
    close(sockfd);

    return (result == 0) ? 1 : 0; // Returns 1 if open, 0 if closed
}
