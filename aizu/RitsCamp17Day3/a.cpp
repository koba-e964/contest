#include <iostream>
#include <string>
#include <vector>


#define rep(i, n) for (int i = 0; i < int(n); ++i)

using namespace std;

int main() {
  string s;
  cin >> s;
  int a[26] = {};
  rep(i, s.length()) {
    a[s[i] - 'a'] += 1;
  }
  int cnt = 0;
  rep(i, 26) {
    cnt +=
    a[i] % 2;
  }
  cout << cnt / 2 << endl;
}
