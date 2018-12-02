#include <bits/stdc++.h>

using namespace std;

int main(){
    int n;
    cin >> n;
    vector<int> file(n);
    for(int file_i = 0; file_i < n; file_i++){
       cin >> file[file_i];
    }
    int m = 0;
    for (int i = 0; i < n;) {
        int t = file[i];
        i += 1;
        for (int j = 0; j < t; ++j, ++i) {
            
        }
        m += 1;
    }
    cout << m << endl;
    //  Print the number of arrays defined in 'file' to STDOUT.
    return 0;
}
