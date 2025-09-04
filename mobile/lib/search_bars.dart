import 'package:flutter/material.dart';

class SearchBars extends StatefulWidget {
  const SearchBars({super.key});
  @override State<SearchBars> createState()=>_SearchBarsState();
}

class _SearchBarsState extends State<SearchBars> {
  final apps = TextEditingController();
  final leisure = TextEditingController();
  final sites = TextEditingController();

  void go(String type, String q){
    // open API in external browser or in webview screen in a real app
    debugPrint('search $type $q');
  }

  @override
  Widget build(BuildContext ctx){
    return Padding(
      padding: const EdgeInsets.all(8),
      child: Column(
        children: [
          TextField(controller: apps, decoration: const InputDecoration(hintText: 'Apps Web/Android/Linux/Mac')),
          const SizedBox(height: 8),
          TextField(controller: leisure, decoration: const InputDecoration(hintText: 'Loisirs / Achats / Jeux / Design')),
          const SizedBox(height: 8),
          TextField(controller: sites, decoration: const InputDecoration(hintText: 'Sites à usage massif')),
          const SizedBox(height: 8),
          Row(
            children: [
              ElevatedButton(onPressed: ()=>go('apps', apps.text), child: const Text('Chercher')),
              const SizedBox(width: 8),
              ElevatedButton(onPressed: ()=>go('leisure', leisure.text), child: const Text('Chercher')),
              const SizedBox(width: 8),
              ElevatedButton(onPressed: ()=>go('sites', sites.text), child: const Text('Chercher')),
            ],
          )
        ],
      ),
    );
  }
}
