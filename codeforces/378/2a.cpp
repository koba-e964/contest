#include <iostream>
#include <vector>
#include <string>


using namespace std;

int main() {
  string s;
  cin >> s;
  int n = s.length();
  vector<int> t(1, 0);
  string vowel("AEIUOY");
  for (int i = 0; i < n; ++i) {
    char c = s[i];
    if (vowel.find(c) != string::npos) {
      t.push_back(i + 1);
    }
  }
  t.push_back(n + 1);
  int ma = 0;
  for (int i = 0; i < t.size() - 1; ++i) {
    ma = max(ma, t[i + 1] - t[i]);
  }
  cout << ma << endl;
}
