#include <iostream>

using namespace std;

int main() {
    int n;
    cin >> n;
    long long f[3] = {};
    for (int i = 0; i < n; i++) {
        int x;
        cin >> x;
        f[x % 3]++;
    }
    cout << f[0] * (f[0] - 1) / 2 + f[1] * f[2] << endl;
}
