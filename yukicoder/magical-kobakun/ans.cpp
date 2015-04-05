#include <iostream>

using namespace std;

int main(void) {
    int m, n;
    cin >> m >> n;
    int cnt = 0;
    while (n > 0) {
        cnt += m / n + 1;
        int r = m % n;
        m = n;
        n = r;
    }
    cout << (cnt - 2) << endl;
    return 0;
}
