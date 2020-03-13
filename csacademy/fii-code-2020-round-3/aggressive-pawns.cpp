#include <iostream>
#include <algorithm>

using namespace std;

int main() {
    int q;
    cin >> q;
    while (q--) {
    int x1, y1, x2, y2, x3 , y3;
    cin >> x1 >> y1 >> x2 >> y2 >> x3 >> y3;
    int d1 = abs(x1 - x3) + abs(y1 - y3);
    int d2 = max(abs(x2 - x3), abs(y2 - y3));
    cout << (d1 == d2 ? "Same time" : d1 > d2 ? "Second" : "First") << endl;
    }
    return 0;
}
