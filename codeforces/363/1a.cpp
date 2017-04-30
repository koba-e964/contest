#include <iostream>
#include <vector>
#include <string>


using namespace std;

int main() {
  int n;
  cin >> n;
  vector<int> a(n);
  for (int i = 0; i < n; ++i) {
    cin >> a[i];
  }
  for (int i = 0; i < n; i += 2) {
    if (a[i] == 1) {
      a[i] = 2;
    } else if (a[i] == 2) {
      a[i] = 1;
    }
  }
  int cnt = 0;
  int cur = 3;
  for (int i = 0; i < n; ++i) {
    bool ok = true;
    if (cur & a[i]) {
      ok = true;
      cur &= a[i];
    } else {
      ok = false;
      cur = 3;
    }
    cnt += ok ? 1 : 0;
  }
  cout << n - cnt << endl;
}
